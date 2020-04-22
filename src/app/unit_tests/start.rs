use super::*;

#[test]
fn start__app_starts() -> Result<()> {
    // Given
    // Using localhost because we assume it shows up first in the list
    let port = 4444;
    let expected_socket = SocketAddr::from(([127, 0, 0, 1], 4444));
    let sut = App::start;

    // When
    let actual_app = sut(port)?;

    // Then
    assert_eq!(actual_app.local_socket, expected_socket);
    Ok(())
}
