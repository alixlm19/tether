---
name: ai-engineering
description: >-
  Use this skill when working on LLM integrations, semantic caching, vector embeddings (e.g., fastembed), and AI proxy architecture.
---

# AI Engineering Guidelines

When working on AI, RAG pipelines, or LLM integrations in this workspace, follow these community-curated best practices for AI engineers:

## 1. Prevent Hallucinations & Use Exact Schemas
- Do not hallucinate SDK methods or use deprecated API endpoints. 
- This project acts as a transparent proxy. Always adhere strictly to the exact target API schemas (e.g., OpenAI's `/v1/chat/completions`). 
- When returning cached responses, ensure the payload exactly mimics a live LLM response (including `id`, `object`, `created`, `model`, and `choices`).

## 2. Semantic Caching & Vector Mathematics
- **Embeddings:** Understand that user prompts are converted into dense vectors. For `fastembed` (BAAI/bge-small-en-v1.5), ensure exactly **384-dimensional arrays** are used.
- **Distance Metrics:** When matching vectors, explicitly handle distance thresholds (like cosine distance) to distinguish between a valid cache hit and a cache miss.

## 3. Tool Calling vs. Model Output
- Differentiate clearly between final natural language responses and intermediate tool calls. 
- When designing caching strategies, account for volatile tool outputs (which shouldn't be cached) versus deterministic tool invocation (which can be cached).

## 4. Managing Complexity (Tokens & Streaming)
- **Token Counting:** Keep the system context window limits in mind. Avoid injecting unnecessary tokens into the proxy payload.
- **Streaming:** Be highly aware of async streaming patterns. If a request demands `stream: true`, you must handle Server-Sent Events (SSE) properly.
- **Role Discipline:** Maintain strict boundaries between `user`, `assistant`, `system`, and `tool` roles in the message history.
