<?php

final class A
{
    protected int $value {
        get => 1;
    }
}
-----
<?php

final class A
{
    private int $value {
        get => 1;
    }
}
