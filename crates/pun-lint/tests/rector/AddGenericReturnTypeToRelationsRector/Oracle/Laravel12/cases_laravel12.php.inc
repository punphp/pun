<?php

namespace App\Models {

    use Illuminate\Database\Eloquent\Model;
    use Illuminate\Database\Eloquent\Relations\BelongsTo;
    use Illuminate\Database\Eloquent\Relations\BelongsToMany;
    use Illuminate\Database\Eloquent\Relations\HasMany;

    class Account extends Model {}

    class User extends Model
    {
        public function accounts(): HasMany
        {
            return $this->hasMany(Account::class);
        }

        /**
         * The roles.
         */
        public function roles(): BelongsToMany
        {
            return $this->belongsToMany(Account::class);
        }

        /* plain */
        public function plain(): HasMany
        {
            return $this->hasMany(Account::class);
        }

        /**
       * Bad indent.
           */
        public function badIndent(): HasMany
        {
            return $this->hasMany(Account::class);
        }

        public function morph(): BelongsTo
        {
            return $this->morphTo(Account::class);
        }

        public function named(): HasMany
        {
            return $this->hasMany(related: Account::class);
        }

        public function spread(): HasMany
        {
            $args = [Account::class];

            return $this->hasMany(...$args);
        }

        public function noValue(): HasMany
        {
            return;
        }
    }
}
-----
<?php

namespace App\Models {

    use Illuminate\Database\Eloquent\Model;
    use Illuminate\Database\Eloquent\Relations\BelongsTo;
    use Illuminate\Database\Eloquent\Relations\BelongsToMany;
    use Illuminate\Database\Eloquent\Relations\HasMany;

    class Account extends Model {}

    class User extends Model
    {
        /**
         * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
         */
        public function accounts(): HasMany
        {
            return $this->hasMany(Account::class);
        }

        /**
         * The roles.
         * @return \Illuminate\Database\Eloquent\Relations\BelongsToMany<\App\Models\Account, $this, \Illuminate\Database\Eloquent\Relations\Pivot>
         */
        public function roles(): BelongsToMany
        {
            return $this->belongsToMany(Account::class);
        }

        /* plain */
        /**
         * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
         */
        public function plain(): HasMany
        {
            return $this->hasMany(Account::class);
        }

        /**
         * Bad indent.
         * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
         */
        public function badIndent(): HasMany
        {
            return $this->hasMany(Account::class);
        }

        /**
         * @return \Illuminate\Database\Eloquent\Relations\BelongsTo<\App\Models\Account, $this>
         */
        public function morph(): BelongsTo
        {
            return $this->morphTo(Account::class);
        }

        /**
         * @return \Illuminate\Database\Eloquent\Relations\HasMany<\App\Models\Account, $this>
         */
        public function named(): HasMany
        {
            return $this->hasMany(related: Account::class);
        }

        public function spread(): HasMany
        {
            $args = [Account::class];

            return $this->hasMany(...$args);
        }

        public function noValue(): HasMany
        {
            return;
        }
    }
}
