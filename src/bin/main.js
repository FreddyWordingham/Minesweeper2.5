export function alert_game_over() {
    console.log("GAME OVER!");
    $("#display").fadeOut();
    location.reload();
    return "Rust";
}
