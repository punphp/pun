<?php

class P
{
    public int $a = 1;
    public int $b = 2;
}

class C extends P
{
    public int $a {
        get => 3;
    }
    public int $b = 4 {
        set => $value;
    }
}
-----
<?php

class P
{
    public int $a = 1;
    public int $b = 2;
}

class C extends P
{
    #[\Override]
    public int $a {
        get => 3;
    }
    #[\Override]
    public int $b = 4 {
        set => $value;
    }
}
