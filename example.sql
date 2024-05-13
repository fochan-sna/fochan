--
-- PostgreSQL database dump
--

-- Dumped from database version 16.3 (Debian 16.3-1.pgdg120+1)
-- Dumped by pg_dump version 16.3 (Debian 16.3-1.pgdg120+1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: uuid-ossp; Type: EXTENSION; Schema: -; Owner: -
--

CREATE EXTENSION IF NOT EXISTS "uuid-ossp" WITH SCHEMA public;


--
-- Name: EXTENSION "uuid-ossp"; Type: COMMENT; Schema: -; Owner: 
--

COMMENT ON EXTENSION "uuid-ossp" IS 'generate universally unique identifiers (UUIDs)';


--
-- Name: diesel_manage_updated_at(regclass); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_manage_updated_at(_tbl regclass) RETURNS void
    LANGUAGE plpgsql
    AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$;


ALTER FUNCTION public.diesel_manage_updated_at(_tbl regclass) OWNER TO postgres;

--
-- Name: diesel_set_updated_at(); Type: FUNCTION; Schema: public; Owner: postgres
--

CREATE FUNCTION public.diesel_set_updated_at() RETURNS trigger
    LANGUAGE plpgsql
    AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$;


ALTER FUNCTION public.diesel_set_updated_at() OWNER TO postgres;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: messages; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.messages (
    message_id integer NOT NULL,
    topic_id uuid NOT NULL,
    user_id uuid NOT NULL,
    content text NOT NULL,
    sent_at timestamp without time zone DEFAULT CURRENT_TIMESTAMP NOT NULL
);


ALTER TABLE public.messages OWNER TO postgres;

--
-- Name: messages_message_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.messages_message_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER SEQUENCE public.messages_message_id_seq OWNER TO postgres;

--
-- Name: messages_message_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.messages_message_id_seq OWNED BY public.messages.message_id;


--
-- Name: topics; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.topics (
    topic_id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    name text NOT NULL,
    description text NOT NULL
);


ALTER TABLE public.topics OWNER TO postgres;

--
-- Name: users; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.users (
    user_id uuid DEFAULT public.uuid_generate_v4() NOT NULL,
    username text NOT NULL
);


ALTER TABLE public.users OWNER TO postgres;

--
-- Name: messages message_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.messages ALTER COLUMN message_id SET DEFAULT nextval('public.messages_message_id_seq'::regclass);


--
-- Data for Name: messages; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.messages (message_id, topic_id, user_id, content, sent_at) FROM stdin;
\.


--
-- Data for Name: topics; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.topics (topic_id, name, description) FROM stdin;
b1074c65-6006-4858-a8b0-f6ff98b7fe03	Cooking	Topic for cooking
9e1c2867-2ccb-4cef-9066-3b0c96a2d06a	Cars	Topic about racing
f8e3f824-634a-483f-955f-86c998149cab	Anime	Anime is life
\.


--
-- Data for Name: users; Type: TABLE DATA; Schema: public; Owner: postgres
--

COPY public.users (user_id, username) FROM stdin;
\.


--
-- Name: messages_message_id_seq; Type: SEQUENCE SET; Schema: public; Owner: postgres
--

SELECT pg_catalog.setval('public.messages_message_id_seq', 1, false);


--
-- Name: messages messages_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.messages
    ADD CONSTRAINT messages_pkey PRIMARY KEY (message_id);


--
-- Name: topics topics_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.topics
    ADD CONSTRAINT topics_pkey PRIMARY KEY (topic_id);


--
-- Name: users users_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (user_id);


--
-- Name: messages messages_topic_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.messages
    ADD CONSTRAINT messages_topic_id_fkey FOREIGN KEY (topic_id) REFERENCES public.topics(topic_id);


--
-- Name: messages messages_user_id_fkey; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.messages
    ADD CONSTRAINT messages_user_id_fkey FOREIGN KEY (user_id) REFERENCES public.users(user_id);


--
-- PostgreSQL database dump complete
--

