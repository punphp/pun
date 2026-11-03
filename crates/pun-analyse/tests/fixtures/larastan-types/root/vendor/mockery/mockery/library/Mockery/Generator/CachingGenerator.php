<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator;

use Override;

use function array_key_exists;

class CachingGenerator implements Generator
{
    /**
     * @var array<string,string>
     */
    protected $cache = [];

    /**
     * @var Generator
     */
    protected $generator;

    public function __construct(Generator $generator)
    {}

    /**
     * @return string
     */
    #[Override]
    public function generate(MockConfiguration $config)
    {}
}
