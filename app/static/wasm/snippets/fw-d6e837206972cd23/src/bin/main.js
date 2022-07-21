export function alert_game_over() {
    console.log("GAME OVER!");
    $("#display").fadeOut();
    setTimeout(() => {
        location.reload();
    }, 500);
    return "Rust";
}
