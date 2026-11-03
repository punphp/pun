<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Framework\MockObject;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This trait is not covered by the backward compatibility promise for PHPUnit
 */
trait StubApi
{
    private readonly TestDoubleState $__phpunit_state;

    public function __phpunit_state(): TestDoubleState
    {}

    public function __phpunit_getInvocationHandler(): InvocationHandler
    {}

    public function __phpunit_unsetInvocationMocker(): void
    {}
}
