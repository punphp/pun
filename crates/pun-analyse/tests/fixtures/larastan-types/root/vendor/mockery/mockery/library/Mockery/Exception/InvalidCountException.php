<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Exception;

use Mockery\CountValidator\Exception;
use Mockery\LegacyMockInterface;

use function in_array;

class InvalidCountException extends Exception
{
    /**
     * @var int|null
     */
    protected $actual;

    /**
     * @var int
     */
    protected $expected = 0;

    /**
     * @var string
     */
    protected $expectedComparative = '<=';

    /**
     * @var string|null
     */
    protected $method;

    /**
     * @var LegacyMockInterface|null
     */
    protected $mockObject;

    /**
     * @return int|null
     */
    public function getActualCount()
    {}

    /**
     * @return int
     */
    public function getExpectedCount()
    {}

    /**
     * @return string
     */
    public function getExpectedCountComparative()
    {}

    /**
     * @return string|null
     */
    public function getMethodName()
    {}

    /**
     * @return LegacyMockInterface|null
     */
    public function getMock()
    {}

    /**
     * @return string|null
     *
     * @throws RuntimeException
     */
    public function getMockName()
    {}

    /**
     * @param  int  $count
     * @return self
     */
    public function setActualCount($count)
    {}

    /**
     * @param  int  $count
     * @return self
     */
    public function setExpectedCount($count)
    {}

    /**
     * @param  string $comp
     * @return self
     */
    public function setExpectedCountComparative($comp)
    {}

    /**
     * @param  string $name
     * @return self
     */
    public function setMethodName($name)
    {}

    /**
     * @return self
     */
    public function setMock(LegacyMockInterface $mock)
    {}
}
