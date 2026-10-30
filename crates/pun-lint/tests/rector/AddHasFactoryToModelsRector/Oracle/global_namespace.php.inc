<?php

use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
}

final class Comment extends Model implements JsonSerializable
{
    public function jsonSerialize(): mixed
    {
        return [];
    }
}
-----
<?php

use Illuminate\Database\Eloquent\Model;

class Post extends Model
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
}

final class Comment extends Model implements JsonSerializable
{
    use \Illuminate\Database\Eloquent\Factories\HasFactory;
    public function jsonSerialize(): mixed
    {
        return [];
    }
}
