<?php

/**
 * This file is part of the ramsey/uuid library
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 *
 * @copyright Copyright (c) Ben Ramsey <ben@benramsey.com>
 * @license http://opensource.org/licenses/MIT MIT
 */

declare(strict_types=1);

namespace Ramsey\Uuid;

use Ramsey\Uuid\Builder\FallbackBuilder;
use Ramsey\Uuid\Builder\UuidBuilderInterface;
use Ramsey\Uuid\Codec\CodecInterface;
use Ramsey\Uuid\Codec\GuidStringCodec;
use Ramsey\Uuid\Codec\StringCodec;
use Ramsey\Uuid\Converter\Number\GenericNumberConverter;
use Ramsey\Uuid\Converter\NumberConverterInterface;
use Ramsey\Uuid\Converter\Time\GenericTimeConverter;
use Ramsey\Uuid\Converter\Time\PhpTimeConverter;
use Ramsey\Uuid\Converter\TimeConverterInterface;
use Ramsey\Uuid\Generator\DceSecurityGenerator;
use Ramsey\Uuid\Generator\DceSecurityGeneratorInterface;
use Ramsey\Uuid\Generator\NameGeneratorFactory;
use Ramsey\Uuid\Generator\NameGeneratorInterface;
use Ramsey\Uuid\Generator\PeclUuidNameGenerator;
use Ramsey\Uuid\Generator\PeclUuidRandomGenerator;
use Ramsey\Uuid\Generator\PeclUuidTimeGenerator;
use Ramsey\Uuid\Generator\RandomGeneratorFactory;
use Ramsey\Uuid\Generator\RandomGeneratorInterface;
use Ramsey\Uuid\Generator\TimeGeneratorFactory;
use Ramsey\Uuid\Generator\TimeGeneratorInterface;
use Ramsey\Uuid\Generator\UnixTimeGenerator;
use Ramsey\Uuid\Guid\GuidBuilder;
use Ramsey\Uuid\Math\BrickMathCalculator;
use Ramsey\Uuid\Math\CalculatorInterface;
use Ramsey\Uuid\Nonstandard\UuidBuilder as NonstandardUuidBuilder;
use Ramsey\Uuid\Provider\Dce\SystemDceSecurityProvider;
use Ramsey\Uuid\Provider\DceSecurityProviderInterface;
use Ramsey\Uuid\Provider\Node\FallbackNodeProvider;
use Ramsey\Uuid\Provider\Node\RandomNodeProvider;
use Ramsey\Uuid\Provider\Node\SystemNodeProvider;
use Ramsey\Uuid\Provider\NodeProviderInterface;
use Ramsey\Uuid\Provider\Time\SystemTimeProvider;
use Ramsey\Uuid\Provider\TimeProviderInterface;
use Ramsey\Uuid\Rfc4122\UuidBuilder as Rfc4122UuidBuilder;
use Ramsey\Uuid\Validator\GenericValidator;
use Ramsey\Uuid\Validator\ValidatorInterface;

use const PHP_INT_SIZE;

/**
 * FeatureSet detects and exposes available features in the current environment
 *
 * A feature set is used by UuidFactory to determine the available features and capabilities of the environment.
 */
class FeatureSet
{
    private ?TimeProviderInterface $timeProvider = null;
    private CalculatorInterface $calculator;
    private CodecInterface $codec;
    private DceSecurityGeneratorInterface $dceSecurityGenerator;
    private NameGeneratorInterface $nameGenerator;
    private NodeProviderInterface $nodeProvider;
    private NumberConverterInterface $numberConverter;
    private RandomGeneratorInterface $randomGenerator;
    private TimeConverterInterface $timeConverter;
    private TimeGeneratorInterface $timeGenerator;
    private TimeGeneratorInterface $unixTimeGenerator;
    private UuidBuilderInterface $builder;
    private ValidatorInterface $validator;

    /**
     * @param bool $useGuids True build UUIDs using the GuidStringCodec
     * @param bool $force32Bit True to force the use of 32-bit functionality (primarily for testing purposes)
     * @param bool $forceNoBigNumber (obsolete)
     * @param bool $ignoreSystemNode True to disable attempts to check for the system node ID (primarily for testing purposes)
     * @param bool $enablePecl True to enable the use of the PeclUuidTimeGenerator to generate version 1 UUIDs
     *
     * @phpstan-ignore constructor.unusedParameter ($forceNoBigNumber is deprecated)
     */
    public function __construct(
        bool $useGuids = false,
        private bool $force32Bit = false,
        bool $forceNoBigNumber = false,
        private bool $ignoreSystemNode = false,
        private bool $enablePecl = false,
    ) {}

    /**
     * Returns the builder configured for this environment
     */
    public function getBuilder(): UuidBuilderInterface
    {}

    /**
     * Returns the calculator configured for this environment
     */
    public function getCalculator(): CalculatorInterface
    {}

    /**
     * Returns the codec configured for this environment
     */
    public function getCodec(): CodecInterface
    {}

    /**
     * Returns the DCE Security generator configured for this environment
     */
    public function getDceSecurityGenerator(): DceSecurityGeneratorInterface
    {}

    /**
     * Returns the name generator configured for this environment
     */
    public function getNameGenerator(): NameGeneratorInterface
    {}

    /**
     * Returns the node provider configured for this environment
     */
    public function getNodeProvider(): NodeProviderInterface
    {}

    /**
     * Returns the number converter configured for this environment
     */
    public function getNumberConverter(): NumberConverterInterface
    {}

    /**
     * Returns the random generator configured for this environment
     */
    public function getRandomGenerator(): RandomGeneratorInterface
    {}

    /**
     * Returns the time converter configured for this environment
     */
    public function getTimeConverter(): TimeConverterInterface
    {}

    /**
     * Returns the time generator configured for this environment
     */
    public function getTimeGenerator(): TimeGeneratorInterface
    {}

    /**
     * Returns the Unix Epoch time generator configured for this environment
     */
    public function getUnixTimeGenerator(): TimeGeneratorInterface
    {}

    /**
     * Returns the validator configured for this environment
     */
    public function getValidator(): ValidatorInterface
    {}

    /**
     * Sets the calculator to use in this environment
     */
    public function setCalculator(CalculatorInterface $calculator): void
    {}

    /**
     * Sets the DCE Security provider to use in this environment
     */
    public function setDceSecurityProvider(DceSecurityProviderInterface $dceSecurityProvider): void
    {}

    /**
     * Sets the node provider to use in this environment
     */
    public function setNodeProvider(NodeProviderInterface $nodeProvider): void
    {}

    /**
     * Sets the time provider to use in this environment
     */
    public function setTimeProvider(TimeProviderInterface $timeProvider): void
    {}

    /**
     * Set the validator to use in this environment
     */
    public function setValidator(ValidatorInterface $validator): void
    {}

    /**
     * Returns a codec configured for this environment
     *
     * @param bool $useGuids Whether to build UUIDs using the GuidStringCodec
     */
    private function buildCodec(bool $useGuids = false): CodecInterface
    {}

    /**
     * Returns a DCE Security generator configured for this environment
     */
    private function buildDceSecurityGenerator(
        DceSecurityProviderInterface $dceSecurityProvider,
    ): DceSecurityGeneratorInterface {}

    /**
     * Returns a node provider configured for this environment
     */
    private function buildNodeProvider(): NodeProviderInterface
    {}

    /**
     * Returns a number converter configured for this environment
     */
    private function buildNumberConverter(CalculatorInterface $calculator): NumberConverterInterface
    {}

    /**
     * Returns a random generator configured for this environment
     */
    private function buildRandomGenerator(): RandomGeneratorInterface
    {}

    /**
     * Returns a time generator configured for this environment
     *
     * @param TimeProviderInterface $timeProvider The time provider to use with
     *     the time generator
     */
    private function buildTimeGenerator(TimeProviderInterface $timeProvider): TimeGeneratorInterface
    {}

    /**
     * Returns a Unix Epoch time generator configured for this environment
     */
    private function buildUnixTimeGenerator(): TimeGeneratorInterface
    {}

    /**
     * Returns a name generator configured for this environment
     */
    private function buildNameGenerator(): NameGeneratorInterface
    {}

    /**
     * Returns a time converter configured for this environment
     */
    private function buildTimeConverter(CalculatorInterface $calculator): TimeConverterInterface
    {}

    /**
     * Returns a UUID builder configured for this environment
     *
     * @param bool $useGuids Whether to build UUIDs using the GuidStringCodec
     */
    private function buildUuidBuilder(bool $useGuids = false): UuidBuilderInterface
    {}

    /**
     * Returns true if the PHP build is 64-bit
     */
    private function is64BitSystem(): bool
    {}
}
