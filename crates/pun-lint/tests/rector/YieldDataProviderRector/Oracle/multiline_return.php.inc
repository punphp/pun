<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class MultilineTest extends TestCase
{
    #[DataProvider('flatUnion')]
    #[DataProvider('parenUnion')]
    #[DataProvider('parenNonArray')]
    #[DataProvider('parenFirst')]
    #[DataProvider('nextLine')]
    #[DataProvider('trailingSpace')]
    #[DataProvider('removeNextLine')]
    #[DataProvider('removeTrailingSpace')]
    #[DataProvider('sameAndNext')]
    #[DataProvider('afterBlank')]
    #[DataProvider('removeAfterBlank')]
    public function testIt($a)
    {
    }

    /**
     * @return array|int|string
     */
    public static function flatUnion()
    {
        return [[1]];
    }

    /**
     * @return array|(int|string)
     */
    public static function parenUnion()
    {
        return [[1]];
    }

    /**
     * @return int|(string|bool)
     */
    public static function parenNonArray()
    {
        return [[1]];
    }

    /**
     * @return (int|string)|array
     */
    public static function parenFirst()
    {
        return [[1]];
    }

    /**
     * @return array
     * more description
     */
    public static function nextLine()
    {
        return [[1]];
    }

    /**
     * @return array 
     *   more
     */
    public static function trailingSpace()
    {
        return [[1]];
    }

    /**
     * @return int
     * more
     */
    public static function removeNextLine()
    {
        return [[1]];
    }

    /**
     * @return int 
     * more
     */
    public static function removeTrailingSpace()
    {
        return [[1]];
    }

    /**
     * @return array desc
     * more
     */
    public static function sameAndNext()
    {
        return [[1]];
    }

    /**
     * @return array
     *
     * text after a blank line
     */
    public static function afterBlank()
    {
        return [[1]];
    }

    /**
     * @return int
     *
     * text after a blank line
     */
    public static function removeAfterBlank()
    {
        return [[1]];
    }
}
-----
<?php

namespace Pun\Yd;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\TestCase;

final class MultilineTest extends TestCase
{
    #[DataProvider('flatUnion')]
    #[DataProvider('parenUnion')]
    #[DataProvider('parenNonArray')]
    #[DataProvider('parenFirst')]
    #[DataProvider('nextLine')]
    #[DataProvider('trailingSpace')]
    #[DataProvider('removeNextLine')]
    #[DataProvider('removeTrailingSpace')]
    #[DataProvider('sameAndNext')]
    #[DataProvider('afterBlank')]
    #[DataProvider('removeAfterBlank')]
    public function testIt($a)
    {
    }

    public static function flatUnion(): \Iterator
    {
        yield [1];
    }

    public static function parenUnion(): \Iterator
    {
        yield [1];
    }

    public static function parenNonArray(): \Iterator
    {
        yield [1];
    }

    public static function parenFirst(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int|string, mixed>
     * more description
     */
    public static function nextLine(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int|string, mixed>
     *   more
     */
    public static function trailingSpace(): \Iterator
    {
        yield [1];
    }

    public static function removeNextLine(): \Iterator
    {
        yield [1];
    }

    public static function removeTrailingSpace(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int|string, mixed> desc
     * more
     */
    public static function sameAndNext(): \Iterator
    {
        yield [1];
    }

    /**
     * @return \Iterator<int|string, mixed>
     *
     * text after a blank line
     */
    public static function afterBlank(): \Iterator
    {
        yield [1];
    }

    public static function removeAfterBlank(): \Iterator
    {
        yield [1];
    }
}
