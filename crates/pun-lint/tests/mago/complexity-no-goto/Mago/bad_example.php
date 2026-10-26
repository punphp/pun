<?php

$i = 0;
loop:
if ($i >= 10) {
    goto end;
}

$i++;
goto loop;
end:
