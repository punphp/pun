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

use PHPUnit\Event\Facade as EventFacade;
use PHPUnit\Framework\Exception;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class InvocationMockerImplementation extends AbstractInvocationImplementation implements InvocationMocker
{
    /**
     * @throws Exception
     * @throws MethodNameNotConfiguredException
     * @throws MethodParametersAlreadyConfiguredException
     * @throws MethodParametersAlreadyConfiguredForAnotherMatcherException
     *
     * @return $this
     */
    public function with(mixed ...$arguments): InvocationMocker
    {}

    public function withParameterSetsInOrder(mixed ...$arguments): InvocationMocker
    {}

    public function withParameterSetsInAnyOrder(mixed ...$arguments): InvocationMocker
    {}

    public function withParameterSetsInPartialOrder(mixed ...$arguments): InvocationMocker
    {}

    /**
     * @throws MethodNameNotConfiguredException
     * @throws MethodParametersAlreadyConfiguredException
     * @throws MethodParametersAlreadyConfiguredForAnotherMatcherException
     *
     * @return $this
     */
    public function withAnyParameters(): InvocationMocker
    {}

    /**
     * @param non-empty-string $id
     *
     * @throws MatcherAlreadyRegisteredException
     *
     * @return $this
     *
     * @deprecated https://github.com/sebastianbergmann/phpunit/issues/6537
     */
    public function id(string $id): InvocationMocker
    {}

    /**
     * @param non-empty-string $id
     *
     * @return $this
     *
     * @deprecated https://github.com/sebastianbergmann/phpunit/issues/6537
     */
    public function after(string $id): InvocationMocker
    {}

    /**
     * @throws MethodParametersAlreadyConfiguredForAnotherMatcherException
     */
    private function ensureNoOtherMatcherHasParametersRuleForSameMethod(): void
    {}

    private function emitDeprecationWhenCreatedWithoutExplicitExpects(): void
    {}
}
