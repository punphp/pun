<?php
interface Contract {}
function right(Contract&int $value): Contract&int { return $value; }
function left(int&Contract $value): int&Contract { return $value; }
function nullable((Contract&int)|null $value): (Contract&int)|null { return $value; }
function otherArm((Contract&int)|string $value): (Contract&int)|string { return $value; }
function nested((Contract&int&Countable)|null $value): (Contract&int&Countable)|null { return $value; }
function valid(Contract&Countable $value): Contract&Countable { return $value; }
function after(int $value): string { missing_control(); return 'after'; }
