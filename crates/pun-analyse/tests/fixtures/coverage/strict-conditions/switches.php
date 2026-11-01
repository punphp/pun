<?php declare(strict_types = 1);

namespace StrictConditions;

const ANSWER = 42;

enum Status
{
    case Active;
    case Closed;
}

class Switches
{
    public const NAME = 'name';
    public const COUNT = 3;

    public function switches(int $int, string $string, ?int $nullable, Status $status, int|string $union, bool $bool, mixed $mixed): void
    {
        switch ($int) {
            case 1:
            case '1':
            case 1.5:
            case ANSWER:
            case self::NAME:
            case self::COUNT:
            case \PHP_INT_MAX:
            case null:
            case true:
            case [1]:
            default:
        }
        switch ($string) {
            case 'a':
            case "b{$int}":
            case 2:
            case Switches::NAME:
            case \StrictConditions\Switches::COUNT:
                break;
        }
        switch ($nullable) {
            case null:
            case 1:
            case 'x':
        }
        switch ($status) {
            case Status::Active:
            case Status::Closed:
            case 'active':
        }
        switch ($union) {
            case 1:
            case 'a':
            case 1.0:
        }
        switch (true) {
            case $int > 1:
            case $string:
            case $bool:
        }
        switch ($mixed) {
            case 1:
            case 'a':
        }
        switch ($int + 1) {
            case -1:
            case +2:
            case 1 + 2:
            case 0x1F:
            case 0b11:
            case 1_000:
            case 'x' . 'y':
            case \strlen($string):
            case $this->name():
        }
    }

    public function name(): string
    {
        return self::NAME;
    }
}
