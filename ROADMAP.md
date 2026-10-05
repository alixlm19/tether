# Tether Roadmap & Architecture Vision

Tether is currently in active development. While the core MVP focuses on intercepting requests and providing baseline semantic caching, the long-term vision is to make Tether the ultimate companion proxy for agentic frameworks (like `pydantic-ai`, LangChain, etc.).

Below are the planned features and architectural concepts that will guide future development.

## 1. Advanced Caching Strategies (Header-Based)

Agentic workflows require more granular control than standard "cache everything" proxies. To support this, Tether will allow clients to pass custom instructions via HTTP headers (e.g., `X-Tether-Strategy`), which Tether will read, strip, and apply before forwarding the request to the upstream LLM.

### Planned Strategies:

- **`cache_tool_calls_only`** (The "Weather" Problem)
  - **Concept:** Caching the final output of volatile data (like a weather API) is dangerous. However, caching the LLM's *decision* to call a tool is highly deterministic and safe.
  - **Behavior:** Tether will cache the initial tool call and extracted arguments. When the client executes the tool and sends the fresh data back, Tether bypasses the cache, forcing the LLM to generate a real-time final answer based on the new data.

- **`cache_all`** (Default)
  - Caches both intermediate tool calls and final natural language responses. Best for static queries.

- **`bypass`**
  - Completely bypasses the cache for highly sensitive or strictly real-time queries.

## 2. Cache Invalidation & Tagging

- **Time-to-Live (TTL):** Allow clients to specify `X-Tether-TTL` to automatically expire cache entries after a certain duration (e.g., 3600 seconds for weather, 24 hours for database lookups).
- **Tag-Based Eviction:** Allow clients to tag requests (`X-Tether-Tags: user_123, billing`). This allows for targeted cache invalidation without flushing the entire vector database (e.g., when a user updates their billing profile, all related cached generations are purged).

## 3. Streaming Support

Agent frameworks rely heavily on streaming (`stream: true`) for responsive UIs.
- Tether must be able to gracefully intercept streaming requests.
- On a cache hit, Tether will simulate the upstream API's streaming chunk format, replaying the cached response seamlessly so the downstream client framework does not break.

## 4. Multi-Provider Support

While initially targeting the OpenAI API format (which is the de facto standard for many agent frameworks), Tether should eventually support transparent routing and caching for Anthropic, Gemini, and local models via Ollama.
