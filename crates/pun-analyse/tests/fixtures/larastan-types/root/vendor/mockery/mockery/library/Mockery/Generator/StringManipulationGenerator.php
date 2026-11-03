<?php

/**
 * Mockery (https://docs.mockery.io/en/stable/)
 *
 * @copyright https://github.com/mockery/mockery/blob/HEAD/COPYRIGHT.md
 * @license   https://github.com/mockery/mockery/blob/HEAD/LICENSE BSD 3-Clause License
 * @see       https://github.com/mockery/mockery for the canonical source repository
 */

namespace Mockery\Generator;

use Mockery\Exception;
use Mockery\Generator\StringManipulation\Pass\AvoidMethodClashPass;
use Mockery\Generator\StringManipulation\Pass\CallTypeHintPass;
use Mockery\Generator\StringManipulation\Pass\ClassAttributesPass;
use Mockery\Generator\StringManipulation\Pass\ClassNamePass;
use Mockery\Generator\StringManipulation\Pass\ClassPass;
use Mockery\Generator\StringManipulation\Pass\ConstantsPass;
use Mockery\Generator\StringManipulation\Pass\InstanceMockPass;
use Mockery\Generator\StringManipulation\Pass\InterfacePass;
use Mockery\Generator\StringManipulation\Pass\MagicMethodTypeHintsPass;
use Mockery\Generator\StringManipulation\Pass\MethodDefinitionPass;
use Mockery\Generator\StringManipulation\Pass\Pass;
use Mockery\Generator\StringManipulation\Pass\RemoveBuiltinMethodsThatAreFinalPass;
use Mockery\Generator\StringManipulation\Pass\RemoveDestructorPass;
use Mockery\Generator\StringManipulation\Pass\RemoveUnserializeForInternalSerializableClassesPass;
use Mockery\Generator\StringManipulation\Pass\TraitPass;
use Override;

use function file_get_contents;

class StringManipulationGenerator implements Generator
{
    /**
     * @var list<Pass>
     */
    protected $passes = [];

    /**
     * @var string
     */
    private $code;

    /**
     * @param list<Pass> $passes
     */
    public function __construct(array $passes)
    {}

    /**
     * @return void
     */
    public function addPass(Pass $pass)
    {}

    /**
     * @return MockDefinition
     *
     * @throws Exception
     */
    #[Override]
    public function generate(MockConfiguration $config)
    {}

    /**
     * Creates a new StringManipulationGenerator with the default passes
     *
     * @return static
     */
    public static function withDefaultPasses()
    {}
}
