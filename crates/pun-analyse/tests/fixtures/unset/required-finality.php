<?php
namespace UnsetRequiredFinality;

/** @property int $implicit
 * @property int $explicit
 * @property int $sibling
 * @property int $promoted
 * @property int $commented
 */
#[\AllowDynamicProperties]
class Overlay
{
    private int $implicit = 1;
    private private(set) int $explicit = 1, $sibling = 2;
    private /* private(set) */ int $commented = 1;
    public function __construct(private private(set) int $promoted = 1) {}
}
/** @phpstan-require-extends Overlay */
interface Required {}
function properties(Overlay $class, Required $required): void
{
    unset($class->implicit);
    unset($class->explicit);
    unset($class->sibling);
    unset($class->promoted);
    unset($class->commented);
    unset($required->implicit);
    unset($required->explicit);
    unset($required->sibling);
    unset($required->promoted);
    unset($required->commented);
}
