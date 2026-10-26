<?php
namespace FailedMethodNever;
class Known {
    public function stop(): never { throw new \Exception(); }
}
function observe(Known $known): void {
    \PHPStan\dumpType($known->stop());
}
