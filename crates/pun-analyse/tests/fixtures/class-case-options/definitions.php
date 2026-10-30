<?php
namespace ClassCaseOptions\Definitions;

#[\Attribute]
class Marker
{
    public const VALUE = 1;
    public static int $value = 1;

    public static function make(): self { return new self(); }
}

class Alias {}
class Failure extends \Exception {}
interface Contract {}
trait Feature {}
enum Kind { case One; }
