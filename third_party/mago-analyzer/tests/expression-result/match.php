<?php
function resultVoid(): void {}
function resultMatch(bool $flag): void {
    $all = match ($flag) { true => resultVoid()/* result */, false => resultVoid()/* result */ }/* result */;
    $all/* result */;
}
