<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator\StringManipulation\Pass;

use Mockery\Exception;
use Mockery\Generator\Method;
use Mockery\Generator\MockConfiguration;
use Mockery\Generator\Parameter;
use Mockery\Generator\TargetClassInterface;
use Override;

use function array_filter;
use function array_merge;
use function end;
use function in_array;
use function is_array;
use function preg_match;
use function preg_match_all;
use function preg_replace;
use function rtrim;
use function sprintf;

class MagicMethodTypeHintsPass implements Pass
{
    /**
     * @var array
     */
    private $mockMagicMethods = [
        '__construct',
        '__destruct',
        '__call',
        '__callStatic',
        '__get',
        '__set',
        '__isset',
        '__unset',
        '__sleep',
        '__wakeup',
        '__toString',
        '__invoke',
        '__set_state',
        '__clone',
        '__debugInfo',
    ];

    /**
     * Apply implementation.
     *
     * @param  non-empty-string $code
     * @return non-empty-string
     *
     * @throws Exception
     */
    #[Override]
    public function apply($code, MockConfiguration $config)
    {}

    /**
     * Returns the magic methods within the
     * passed DefinedTargetClass.
     *
     * @return array<Method>
     */
    public function getMagicMethods(?TargetClassInterface $class = null)
    {}

    /**
     * Renders the type hint for the passed parameter.
     *
     * @return string
     */
    protected function renderTypeHint(Parameter $param)
    {}

    /**
     * Applies type hints of magic methods from
     * class to the passed code.
     *
     * @param  non-empty-string $code
     * @return non-empty-string
     */
    private function applyMagicTypeHints($code, Method $method)
    {}

    /**
     * Returns a regex string used to match the
     * declaration of some method.
     *
     * @param  string           $methodName
     * @return non-empty-string
     */
    private function getDeclarationRegex($methodName)
    {}

    /**
     * Gets the declaration code, as a string, for the passed method.
     *
     * @return non-empty-string
     */
    private function getMethodDeclaration(Method $method, array $namedParameters)
    {}

    /**
     * Returns the method original parameters, as they're
     * described in the $code string.
     *
     * @param  non-empty-string $code
     * @return array
     */
    private function getOriginalParameters($code, Method $method)
    {}

    /**
     * Checks if the method is declared within code.
     *
     * @param  non-empty-string $code
     * @return bool
     */
    private function isMethodWithinCode($code, Method $method)
    {}
}
