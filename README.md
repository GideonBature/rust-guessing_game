# Guessing Game

## Algorithm Used In Building the Guessing Game

 - Generate a random number within the range of 1 - 100 inclusive and store in a variable.
 - Get the user input from the Keyboard and store in a mutable variable with string as a placeholder.
 - Convert this input to a number, trim (to eliminate space) and parse (check if it could be converted to a number or not).
 - Compare the player guess and the random number, if guess is less than random number, print to stdout "Too small", if it is greater than random number, print to stdout "Too big" and if they are equal, the player wins.
 - Put step 2 - step 4 inside a loop, so that the player will keep playing untill He wins the game or simply quit it using the ctrl + c keyboard combination.
 - Finally, break out of the loop once the player wins the game.

This game is to used to help explore and further help me understand Rust, how it works. How to handle dependencies using the Cargo.toml file, how to use crates and associated methods and functions.
