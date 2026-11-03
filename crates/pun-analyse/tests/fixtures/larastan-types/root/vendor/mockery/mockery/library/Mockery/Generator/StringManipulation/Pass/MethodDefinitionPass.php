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
use Override;

use const PHP_VERSION_ID;

use function array_values;
use function count;
use function enum_exists;
use function get_class;
use function implode;
use function is_object;
use function preg_match;
use function sprintf;
use function strpos;
use function strrpos;
use function strtolower;
use function substr;

use function var_export;

class MethodDefinitionPass implements Pass
{
    /**
     * @param  string $code
     * @return string
     *
     * @throws Exception
     */
    #[Override]
    public function apply($code, MockConfiguration $config)
    {}

    protected function appendToClass($class, $code)
    {}

    protected function renderParams(Method $method, $config)
    {}

    protected function renderReturnType(Method $method)
    {}

    protected function renderTypeHint(Parameter $param)
    {}

    private function renderMethodBody(Method $method, MockConfiguration $config)
    {}

    private function shouldReturnValue(Method $method): bool
    {}
}
