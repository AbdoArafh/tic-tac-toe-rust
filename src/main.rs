fn main() {
    let mut game = tic_tac_toe::Game::new();
    game.init();
    game.listen_to_messages();
    std::thread::sleep(std::time::Duration::from_millis(1000));

    game.print_status();
}
