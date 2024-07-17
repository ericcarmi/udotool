
simple program to write text and press keys

it accepts one long argument separated by spaces

to write most strings, surround by characters with [] -- this avoids issues with shells and quotes

to write the character ", use the command "quo" (with no quotes around it)

other commands represent key presses that do not write text -- e.g. ctrl-shift-down, ctrl-v

example usage
udotool -t quo [%p] quo [+yu] ctrl-down [%run ] ctrl-v enter

this particular command was the motivation for making something that works on windows (xdotool does not work in windows)

the modal text editor [helix](https://github.com/helix-editor/helix) does not yet have the ability to use the file name in a script, so this is a work around for running a python script

makes the assumption that ipython is opened in a terminal below, ctrl-down is used to navigate to that terminal

