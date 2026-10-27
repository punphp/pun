<?php

// The conversion test supplies independent constructor identities through the metadata API.
function implicitValue(): mixed { throw new Exception(); }
function explicitValue(): mixed { throw new Exception(); }
function nonacceptingValue(): mixed { throw new Exception(); }
function implicitObservation() { PHPStan\dumpType(implicitValue()); }
function explicitObservation() { PHPStan\dumpType(explicitValue()); }
function nonacceptingObservation() { PHPStan\dumpType(nonacceptingValue()); }
