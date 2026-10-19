<?php

for ($i = 0; $i < 3; $i++) {
    break; // The loop never truly iterates, as this break is unconditional.
}
-----
3:1: Loop is unconditionally terminated and will not iterate.
