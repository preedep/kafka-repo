export function checkTokenValidity() {
    const token = localStorage.getItem('token');
    const expirationTime = localStorage.getItem('expirationTime');

    if (!token || !expirationTime) {
        return false;
    }

    if (new Date().getTime() > expirationTime) {
        localStorage.removeItem('token');
        localStorage.removeItem('expirationTime');
        return false;
    }

    return true;
}