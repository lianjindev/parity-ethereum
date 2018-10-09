package io.parity.ethereum;

/**
 * Interface to the Parity client.
 */
public class Parity {
    /**
     * Starts the Parity client with the CLI options passed as an array of strings.
     *
     * Each space-delimited option corresponds to an array entry.
     * For example: `["--port", "12345"]`
     *
     * @param options The CLI options to start Parity with
     */
    public Parity(String[] options) {
        long config = configFromCli(options);
        inner = build(config);
    }

    /** Performs a synchronous RPC query.
     *
     * Note that this will block the current thread until the query is finished. You are
     * encouraged to create a background thread if you don't want to block.
     *
     * @param query The JSON-encoded RPC query to perform
     * @return A JSON-encoded result
     */
    public String rpcQuery(String query) {
        return rpcQueryNative(inner, query);
    }

    @Override
    protected void finalize​() {
        destroy(inner);
    }

    static {
        System.loadLibrary("parity");
    }

    private static native long configFromCli(String[] cliOptions);
    private static native long build(long config);
    private static native void destroy(long inner);
    private static native String rpcQueryNative(long inner, String rpc);

    private long inner;
}
