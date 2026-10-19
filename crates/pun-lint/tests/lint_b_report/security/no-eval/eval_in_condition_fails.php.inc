<?php

if ($needsEval) {
    eval($userInput);
}
-----
4:5: Unsafe use of `eval` construct.
