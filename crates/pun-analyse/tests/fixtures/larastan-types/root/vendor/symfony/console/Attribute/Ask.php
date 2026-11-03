<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Attribute;

use Symfony\Component\Console\Attribute\Reflection\ReflectionMember;
use Symfony\Component\Console\Exception\InvalidArgumentException;
use Symfony\Component\Console\Exception\LogicException;
use Symfony\Component\Console\Input\File\InputFile;
use Symfony\Component\Console\Input\InputInterface;
use Symfony\Component\Console\Question\ConfirmationQuestion;
use Symfony\Component\Console\Question\FileQuestion;
use Symfony\Component\Console\Question\Question;
use Symfony\Component\Console\Style\SymfonyStyle;
use Symfony\Component\Validator\Constraint;

#[\Attribute(\Attribute::TARGET_PARAMETER | \Attribute::TARGET_PROPERTY)]
class Ask implements InteractiveAttributeInterface
{
    public ?\Closure $normalizer;
    public ?\Closure $validator;
    private \Closure $closure;

    /**
     * @param string                     $question    The question to ask the user
     * @param string|bool|int|float|null $default     The default answer to return if the user enters nothing
     * @param bool                       $hidden      Whether the user response must be hidden or not
     * @param bool                       $multiline   Whether the user response should accept newline characters
     * @param bool                       $trimmable   Whether the user response must be trimmed or not
     * @param int|null                   $timeout     The maximum time the user has to answer the question in seconds
     * @param callable|null              $validator   The validator for the question
     * @param int|null                   $maxAttempts The maximum number of attempts allowed to answer the question.
     *                                                Null means an unlimited number of attempts
     */
    public function __construct(
        public string $question,
        public string|bool|int|float|null $default = null,
        public bool $hidden = false,
        public bool $multiline = false,
        public bool $trimmable = true,
        public ?int $timeout = null,
        ?callable $normalizer = null,
        ?callable $validator = null,
        public ?int $maxAttempts = null,
        /** @var Constraint[] */
        public array $constraints = [],
    ) {}

    /**
     * @internal
     */
    public static function tryFrom(\ReflectionParameter|\ReflectionProperty $member, string $name): ?self
    {}

    /**
     * @internal
     */
    public function getFunction(object $instance): \ReflectionFunction
    {}
}
