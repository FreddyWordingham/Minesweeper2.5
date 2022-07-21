export function alert_game_over() {
    console.log("GAME OVER!");
    $("#display").fadeOut();
    setTimeout(function () {
        location.reload();
    }, 1000);
    return "Rust";
}
