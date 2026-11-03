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

namespace Ramsey\Uuid\Provider\Node;

use Ramsey\Uuid\Exception\NodeException;
use Ramsey\Uuid\Provider\NodeProviderInterface;
use Ramsey\Uuid\Type\Hexadecimal;

use function array_filter;
use function array_map;
use function array_walk;
use function count;
use function ob_get_clean;
use function ob_start;
use function preg_match;
use function preg_match_all;
use function reset;
use function str_contains;
use function str_replace;
use function strtolower;
use function strtoupper;
use function substr;

use const GLOB_NOSORT;
use const PREG_PATTERN_ORDER;

/**
 * SystemNodeProvider retrieves the system node ID, if possible
 *
 * The system node ID, or host ID, is often the same as the MAC address for a network interface on the host.
 */
class SystemNodeProvider implements NodeProviderInterface
{
    /**
     * Pattern to match nodes in `ifconfig` and `ipconfig` output.
     */
    private const IFCONFIG_PATTERN = '/[^:]([0-9a-f]{2}([:-])[0-9a-f]{2}(\2[0-9a-f]{2}){4})[^:]/i';

    /**
     * Pattern to match nodes in sysfs stream output.
     */
    private const SYSFS_PATTERN = '/^([0-9a-f]{2}:){5}[0-9a-f]{2}$/i';

    public function getNode(): Hexadecimal
    {}

    /**
     * Returns the system node if found
     */
    protected function getNodeFromSystem(): string
    {}

    /**
     * Returns the network interface configuration for the system
     *
     * @codeCoverageIgnore
     */
    protected function getIfconfig(): string
    {}

    /**
     * Returns MAC address from the first system interface via the sysfs interface
     */
    protected function getSysfs(): string
    {}
}
