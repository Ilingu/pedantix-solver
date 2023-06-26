# PEDANTIX SOLVER

## Little FAQ

### What's this?

Pedantix solver is a script that find the [pedantix](https://cemantix.certitudes.org/pedantix) answer of the day!

> It's done in rust. Therefore it's amazing.

### How does it works?

It goes through a list of words (also known as a [dictionary](https://github.com/frodonh/french-words) 📖) and for each word it checks with the pedantix api if this word in part of the answer.

When the script has all the words it prints the result.

For effectiveness and speed, it spawns a lot of thread (1000 ideally, but in reality 10, see why bellow) that runs in parallel and each one of them will check a precise and unique part of the dictionary.

### Cool, How to use it?

1. `Rule No.1`: Just <ins>**DO NOT.**</ins>
2. `Rule No.2`: Never break `Rule No.1`

Why you may wonder?

Because by doing this your in someway [DDOS](https://www.cloudflare.com/learning/ddos/what-is-a-ddos-attack/)'ing pedantix and that just **not cool**.

In fact this project has the potential to be named "PEDANTIX SOLVER", but may not be. Because there is ~700000 words in the french langage (or more precisly in the dictionary I use) and that doing max 700000 request in less a short amount of time, is like illegal?

But there is still a way, by limiting the amount of request to 10 every 5 seconds, the server should be ok; except that at this rate to test the 700000 words, it'd take ~4days. Needless to say because the puzzle reset everyday it's completly pointless.

## Credit

You can go to the awesome websites of [pedantix](https://cemantix.certitudes.org/pedantix) by David Turner and maintained by [Julie](https://twitter.com/cemantle) and [enigmatix](https://twitter.com/enigmathix).
