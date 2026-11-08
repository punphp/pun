<?php

// A final constant made private loses `final` (PHP rejects a final private constant).
final class A
{
    final protected const FOUR = 4;
}
-----
<?php

// A final constant made private loses `final` (PHP rejects a final private constant).
final class A
{
    private const FOUR = 4;
}
