fun encodeURIComponent(s: String): String {
    val allowedChars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-_.!~*'()"
    val result = StringBuilder()

    for (c in s) {
        if (c in allowedChars) {
            result.append(c)
        } else if (c == ' ') {
            result.append("%20")
        } else {
            val bytes = c.toString().encodeToByteArray()
            for (byte in bytes) {
                val hex = byte.toUByte().toString(16).uppercase().padStart(2, '0')
                result.append("%$hex")
            }
        }
    }

    return result.toString()
}
