<?php

class HookPadding
{
    public int $a {
        get {
            if ($this->b) {
                return 1;
            }
            return 2;
        }
    }

    public bool $b = true;
}
