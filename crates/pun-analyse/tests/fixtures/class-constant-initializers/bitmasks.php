<?php
namespace InitializerBitmasks;
class Values {
    const OR_FLAGS = (true ? 1 : 4) | (true ? 2 : 8);
    const AND_FLAGS = (true ? 1 : 4) & (true ? 2 : 4);
    const XOR_FLAGS = (true ? 1 : 4) ^ (true ? 1 : 8);
    const LEFT = (true ? 1 : 4) << (true ? 1 : 2);
    const RIGHT = (true ? 4 : 16) >> (true ? 1 : 2);
    const RIGHT_BIG = (true ? 4 : 16) >> 64;
    const RIGHT_NEGATIVE = (true ? -1 : -4) >> 64;
    const LEFT_BIG = (true ? 1 : 4) << 64;
}
\PHPStan\dumpType(Values::OR_FLAGS);
\PHPStan\dumpType(Values::AND_FLAGS);
\PHPStan\dumpType(Values::XOR_FLAGS);
\PHPStan\dumpType(Values::LEFT);
\PHPStan\dumpType(Values::RIGHT);
\PHPStan\dumpType(Values::RIGHT_BIG);
\PHPStan\dumpType(Values::RIGHT_NEGATIVE);
\PHPStan\dumpType(Values::LEFT_BIG);

class Keys { const INDEX = 1; }
/** @param array{Keys::INDEX: string} $array */
function keyedShape(array $array): void {
    \PHPStan\dumpType($array);
}
