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

namespace Ramsey\Uuid\Generator;

use Ramsey\Uuid\Converter\NumberConverterInterface;
use Ramsey\Uuid\Exception\DceSecurityException;
use Ramsey\Uuid\Provider\DceSecurityProviderInterface;
use Ramsey\Uuid\Type\Hexadecimal;
use Ramsey\Uuid\Type\Integer as IntegerObject;
use Ramsey\Uuid\Uuid;

use function hex2bin;
use function in_array;
use function pack;
use function str_pad;
use function strlen;
use function substr_replace;

use const STR_PAD_LEFT;

/**
 * DceSecurityGenerator generates strings of binary data based on a local domain, local identifier, node ID, clock
 * sequence, and the current time
 */
class DceSecurityGenerator implements DceSecurityGeneratorInterface
{
    private const DOMAINS = [
        Uuid::DCE_DOMAIN_PERSON,
        Uuid::DCE_DOMAIN_GROUP,
        Uuid::DCE_DOMAIN_ORG,
    ];

    /**
     * Upper bounds for the clock sequence in DCE Security UUIDs.
     */
    private const CLOCK_SEQ_HIGH = 63;

    /**
     * Lower bounds for the clock sequence in DCE Security UUIDs.
     */
    private const CLOCK_SEQ_LOW = 0;

    public function __construct(
        private NumberConverterInterface $numberConverter,
        private TimeGeneratorInterface $timeGenerator,
        private DceSecurityProviderInterface $dceSecurityProvider,
    ) {}

    public function generate(
        int $localDomain,
        ?IntegerObject $localIdentifier = null,
        ?Hexadecimal $node = null,
        ?int $clockSeq = null,
    ): string {}
}
