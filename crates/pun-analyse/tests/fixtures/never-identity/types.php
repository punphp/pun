<?php

function nativeBottom(): never {}
// Reflection accepts these parser types before declaration-validity rules run.
function nativeBottomUnion(): never|never {}
function nativeOrdinary(): never|int {}
function nativeOrdinaryReverse(): int|never {}
function impossibleNative(stdClass&int $value): stdClass&int { return $value; }
/** @return never */
function docBottom() {}
/** @return never|never */
function docUnion() {}
/** @return never|int */
function ordinary() {}
/** @return int|never */
function ordinaryReverse() {}
/** @return never|null */
function nullableBottom() {}

/** @return ($value is true ? never : int) */
function choose(bool $value) { return 1; }
/** @return ($value is true ? int : never) */
function chooseReverse(bool $value) { return 1; }
/** @return ($value is not true ? never : int) */
function chooseNegated(bool $value) { return 1; }
/** @return Closure(bool $later): ($later is true ? never : int) */
function chooseClosure(): Closure { return static fn (bool $later) => 1; }

function selectedNever() { choose(true); } // observe call
function selectedInt() { choose(false); } // observe call
function selectedReverseNever() { chooseReverse(false); } // observe call
function selectedReverseInt() { chooseReverse(true); } // observe call
function selectedNegatedNever() { chooseNegated(false); } // observe call
function selectedNegatedInt() { chooseNegated(true); } // observe call
function selectedBoth(bool $value) { choose($value); } // observe call
function selectedDeferred() { chooseClosure(); } // observe call
