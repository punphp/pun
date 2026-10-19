<?php

final class BranchDim
{
    private array $a = [];

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a[] = 1;
        } else {
            $this->a = [2];
        }
    }
}

final class BranchKey
{
    private array $a = [];

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a['k'] = 1;
        } else {
            $this->a = [2];
        }
    }
}

final class BranchCompound
{
    private int $a = 0;

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a += 1;
        } else {
            $this->a = 2;
        }
    }
}

final class BranchColonElseIf
{
    private int $a = 0;

    public function __construct(int $x)
    {
        if ($x > 1):
            $this->a = 1;
        elseif ($x > 0):
            $this->a = 3;
        else:
            $this->a = 2;
        endif;
    }
}

final class BranchOther
{
    private int $a = 0;

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a = 1;
            echo 1;
        } else {
            $this->a = 2;
        }
    }
}

final class BranchColon
{
    private int $a = 0;

    public function __construct(bool $x)
    {
        if ($x):
            $this->a = 1;
        else:
            $this->a = 2;
        endif;
    }
}
-----
<?php

final class BranchDim
{
    private array $a = [];

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a[] = 1;
        } else {
            $this->a = [2];
        }
    }
}

final class BranchKey
{
    private array $a = [];

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a['k'] = 1;
        } else {
            $this->a = [2];
        }
    }
}

final class BranchCompound
{
    private int $a = 0;

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a += 1;
        } else {
            $this->a = 2;
        }
    }
}

final class BranchColonElseIf
{
    private int $a = 0;

    public function __construct(int $x)
    {
        if ($x > 1):
            $this->a = 1;
        elseif ($x > 0):
            $this->a = 3;
        else:
            $this->a = 2;
        endif;
    }
}

final class BranchOther
{
    private int $a = 0;

    public function __construct(bool $x)
    {
        if ($x) {
            $this->a = 1;
            echo 1;
        } else {
            $this->a = 2;
        }
    }
}

final class BranchColon
{
    private int $a;

    public function __construct(bool $x)
    {
        if ($x):
            $this->a = 1;
        else:
            $this->a = 2;
        endif;
    }
}
