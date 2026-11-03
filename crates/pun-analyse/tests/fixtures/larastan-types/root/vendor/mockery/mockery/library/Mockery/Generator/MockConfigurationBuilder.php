<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator;

use function array_diff;

class MockConfigurationBuilder
{
    /**
     * @var list<string>
     */
    protected $blackListedMethods = [
        '__call',
        '__callStatic',
        '__wakeup',
        '__set',
        '__get',
        '__toString',
        '__isset',
        '__destruct',
        // mocking '__debugInfo' makes it difficult to debug with xdebug
        '__debugInfo',

        // below are reserved words in PHP
        '__halt_compiler', 'abstract', 'and', 'array', 'as',
        'break', 'callable', 'case', 'catch', 'class',
        'clone', 'const', 'continue', 'declare', 'default',
        'die', 'do', 'echo', 'else', 'elseif',
        'empty', 'enddeclare', 'endfor', 'endforeach', 'endif',
        'endswitch', 'endwhile', 'eval', 'exit', 'extends',
        'final', 'for', 'foreach', 'function', 'global',
        'goto', 'if', 'implements', 'include', 'include_once',
        'instanceof', 'insteadof', 'interface', 'isset', 'list',
        'namespace', 'new', 'or', 'print', 'private',
        'protected', 'public', 'require', 'require_once', 'return',
        'static', 'switch', 'throw', 'trait', 'try',
        'unset', 'use', 'var', 'while', 'xor',
    ];

    /**
     * @var array
     */
    protected $constantsMap = [];

    /**
     * @var bool
     */
    protected $instanceMock = false;

    /**
     * @var bool
     */
    protected $mockOriginalDestructor = false;

    /**
     * @var null|string
     */
    protected $name;

    /**
     * @var array
     */
    protected $parameterOverrides = [];

    /**
     * @var list<string>
     */
    protected $php7SemiReservedKeywords = [
        'callable', 'class', 'trait', 'extends', 'implements', 'static', 'abstract', 'final',
        'public', 'protected', 'private', 'const', 'enddeclare', 'endfor', 'endforeach', 'endif',
        'endwhile', 'and', 'global', 'goto', 'instanceof', 'insteadof', 'interface', 'namespace', 'new',
        'or', 'xor', 'try', 'use', 'var', 'exit', 'list', 'clone', 'include', 'include_once', 'throw',
        'array', 'print', 'echo', 'require', 'require_once', 'return', 'else', 'elseif', 'default',
        'break', 'continue', 'switch', 'yield', 'function', 'if', 'endswitch', 'finally', 'for', 'foreach',
        'declare', 'case', 'do', 'while', 'as', 'catch', 'die', 'self', 'parent',
    ];

    /**
     * @var array
     */
    protected $targets = [];

    /**
     * @var array
     */
    protected $whiteListedMethods = [];

    public function __construct()
    {}

    /**
     * @param  string $blackListedMethod
     * @return self
     */
    public function addBlackListedMethod($blackListedMethod)
    {}

    /**
     * @param  list<string> $blackListedMethods
     * @return self
     */
    public function addBlackListedMethods(array $blackListedMethods)
    {}

    /**
     * @param  class-string $target
     * @return self
     */
    public function addTarget($target)
    {}

    /**
     * @param  list<class-string> $targets
     * @return self
     */
    public function addTargets($targets)
    {}

    /**
     * @return self
     */
    public function addWhiteListedMethod($whiteListedMethod)
    {}

    /**
     * @return self
     */
    public function addWhiteListedMethods(array $whiteListedMethods)
    {}

    /**
     * @return MockConfiguration
     */
    public function getMockConfiguration()
    {}

    /**
     * @param  list<string> $blackListedMethods
     * @return self
     */
    public function setBlackListedMethods(array $blackListedMethods)
    {}

    /**
     * @return self
     */
    public function setConstantsMap(array $map)
    {}

    /**
     * @param bool $instanceMock
     */
    public function setInstanceMock($instanceMock)
    {}

    /**
     * @param bool $mockDestructor
     */
    public function setMockOriginalDestructor($mockDestructor)
    {}

    /**
     * @param string $name
     */
    public function setName($name)
    {}

    /**
     * @return self
     */
    public function setParameterOverrides(array $overrides)
    {}

    /**
     * @param  list<string> $whiteListedMethods
     * @return self
     */
    public function setWhiteListedMethods(array $whiteListedMethods)
    {}
}
