use std::{io, time::Duration};

use crossterm::{
    cursor::Hide,
    event::{self, Event, KeyCode},
    terminal, ExecutableCommand,
};
use invaders_2::{
    army::Army,
    frame::{new_frame, Drawable, Transform, Updatable},
    player::Player,
    render,
    sound::Sound,
    status::Status,
};
use rusty_time::timer::Timer;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut audio = Sound::new();

    if let Ok(dir) = std::fs::read_dir("sfx") {
        for entry in dir {
            audio.add(entry.as_ref().unwrap().path())?;
        }
    }

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(terminal::EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Get terminal dimensions
    let (width, height) = terminal::size()?;

    if width < 80 || height < 24 {
        stdout.execute(crossterm::cursor::Show)?;
        stdout.execute(terminal::LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        println!(
            "Terminal needs to be at least 80x24, your terminal is {}x{}",
            width, height
        );
        return Ok(());
    }

    let (render_tx, mut render_rx) = tokio::sync::mpsc::channel(1);

    let render_handle = tokio::spawn(async move {
        let mut last_frame = new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv().await {
                Some(frame) => frame,
                None => break,
            };

            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    let mut instant = Instant::now();

    let mut status = Status::new();
    let mut player = Player::new();
    let mut invaders = Army::new(1);

    let mut death_timer = Timer::from_millis(1000);

    audio.play("startup");

    'gameloop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut frame = new_frame();

        if !player.dead() {
            // Input
            while event::poll(Duration::default())? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('q') | KeyCode::Esc => break 'gameloop,
                        KeyCode::Left | KeyCode::Char('a') => player.move_left(),
                        KeyCode::Right | KeyCode::Char('d') => player.move_right(),
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if player.shoot() {
                                audio.play("pew");
                            }
                        }
                        _ => {}
                    }
                }
            }

            // Update
            let mut updates: Vec<&mut dyn Updatable> = vec![&mut player, &mut invaders];
            for update in &mut updates {
                update.update(delta);
            }

            // Collision detection
            for shot in &mut player.shots {
                if let Some(score) = invaders.check_collision(shot) {
                    status.add_score(score);
                    shot.explode();
                    audio.play("boom");
                }
            }

            for shot in &mut invaders.shots {
                if player.check_collision(shot) {
                    audio.play("boom");
                    shot.explode();
                    match player.die() {
                        None => break 'gameloop,
                        Some(lives) => {
                            death_timer.reset();
                            status.update_lives(lives);
                        }
                    }
                }
            }
        } else {
            death_timer.update(delta);
            if death_timer.ready {
                player.resurrect();
            }
        }

        if invaders.invaded() {
            break 'gameloop;
        }

        if invaders.all_dead() {
            // wait for a bit
            tokio::time::sleep(Duration::from_millis(1000)).await;
            status.level_up();
            invaders = Army::new(status.level);
            player.reset_lives();
            status.update_lives(player.lives);
            player.shots.clear();
            invaders.shots.clear();
        }

        // Draw
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders, &status];
        for drawable in drawables {
            drawable.draw(&mut frame);
        }

        // Render
        let _ = render_tx.send(frame).await;

        tokio::time::sleep(Duration::from_millis(1)).await;
    }

    drop(render_tx);
    let _ = tokio::join!(render_handle);
    audio.wait();
    stdout.execute(crossterm::cursor::Show)?;
    stdout.execute(terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
