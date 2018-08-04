static BC_CONSTEXPR size_t payment_size = 1u + short_hash_size + checksum_size;
typedef byte_array<payment_size> payment;

class BC_API payment_address {

    typedef std::vector<payment_address> list;
    typedef std::shared_ptr<payment_address> ptr;

    /// Extract a payment address list from an input or output script.
    static list extract(const chain::script& script,
        uint8_t p2kh_version=mainnet_p2kh, uint8_t p2sh_version=mainnet_p2sh);

    static list extract_input(const chain::script& script,
        uint8_t p2kh_version=mainnet_p2kh, uint8_t p2sh_version=mainnet_p2sh);
        
    static list extract_output(const chain::script& script,
        uint8_t p2kh_version=mainnet_p2kh, uint8_t p2sh_version=mainnet_p2sh);

    /// Constructors.
    payment_address();
    payment_address(payment_address&& other);
    payment_address(const payment_address& other);
    payment_address(const payment& decoded);
    payment_address(const ec_private& secret);
    payment_address(const std::string& address);
    payment_address(short_hash&& hash, uint8_t version=mainnet_p2kh);
    payment_address(const short_hash& hash, uint8_t version=mainnet_p2kh);
    payment_address(const ec_public& point, uint8_t version=mainnet_p2kh);
    payment_address(const chain::script& script, uint8_t version=mainnet_p2sh);

    /// Operators.
    bool operator<(const payment_address& other) const;
    bool operator==(const payment_address& other) const;
    bool operator!=(const payment_address& other) const;
    payment_address& operator=(const payment_address& other);
    friend std::istream& operator>>(std::istream& in, payment_address& to);
    friend std::ostream& operator<<(std::ostream& out,
        const payment_address& of);

    /// Cast operators.
    operator bool() const;
    operator const short_hash&() const;

    /// Serializer.
    std::string encoded() const;

    /// Accessors.
    uint8_t version() const;
    const short_hash& hash() const;

    /// Methods.
    payment to_payment() const;

private:
    /// Validators.
    static bool is_address(data_slice decoded);

    /// Factories.
    static payment_address from_string(const std::string& address);
    static payment_address from_payment(const payment& decoded);
    static payment_address from_private(const ec_private& secret);
    static payment_address from_public(const ec_public& point, uint8_t version);
    static payment_address from_script(const chain::script& script,
        uint8_t version);

    /// Members.
    /// These should be const, apart from the need to implement assignment.
    bool valid_;
    uint8_t version_;
    short_hash hash_;
};

/// The pre-encoded structure of a payment address or other similar data.
struct BC_API wrapped_data
{
    uint8_t version;
    data_chunk payload;
    uint32_t checksum;
};

} // namespace wallet
} // namespace libbitcoin

// Allow payment_address to be in indexed in std::*map classes.
namespace std
{
template <>
struct hash<bc::wallet::payment_address>
{
    size_t operator()(const bc::wallet::payment_address& address) const
    {
        return std::hash<bc::short_hash>()(address.hash());
    }
};
