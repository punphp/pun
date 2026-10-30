<?php
class DistinctLeft {
    /**
     * @template TLeft
     * @param TLeft $input
     * @return TLeft
     */
    public function invoke($input) { return $input; }
}
class DistinctRight {
    /**
     * @template TRight
     * @param TRight $input
     * @return array{TRight}
     */
    public function invoke($input): array { return [$input]; }
}
function differentTemplates(DistinctLeft|DistinctRight $target): void {
    $i = 0;
    $target->invoke(++$i); // observe
    $i;
}
