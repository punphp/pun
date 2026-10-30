<?php declare(strict_types=1);

namespace DynamicMethodBoundaries;

class Target {
    public function integer(int $value): void {}
    public function sink(): void {}
    public function reference(int &$value): void {}
    /** @no-named-arguments */
    public function noNames(int $value): void {}
    private function hidden(): void {}
    protected function restricted(): void {}

    public function staticClosure(): void {
        $callback = static function (): void {
            $name = 'integer';
            $this->$name();
        };
    }
}

class Value {
    private int $value = 1;
    public function &read(): int { return $this->value; }
}

/** @param array<int, int> $positionals
 * @param array<string, int> $named
 */
function boundaries(Target $target, Value $value, \PDOStatement $statement, array $positionals, array $named): void {
    $target->{
        'integer'
    }();
    $result = $target->{
        'sink'
    }();
    $target->{
        'missing'
    }();
    $name = 'hidden';
    $target->$name();
    $name = 'restricted';
    $target->$name();
    $name = 'INTEGER';
    $target->$name(false);
    $name = 'reference';
    $target->$name($value->read());
    $target->$name(...[$value->read()]);
    $name = 'noNames';
    $target->$name(...$positionals);
    $target->$name(...$named);
    $name = 'fetchAll';
    $statement->$name(\PDO::FETCH_COLUMN, 0);
    $statement->$name(\PDO::FETCH_CLASS, \stdClass::class);
    $statement->$name(\PDO::FETCH_FUNC, 'intval');
    $statement->$name(...$positionals);
    $name = 'missing';
    (new \stdClass)->$name();
}
