--
-- PostgreSQL database dump
--

-- Dumped from database version 11.5
-- Dumped by pg_dump version 11.5

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

SET default_tablespace = '';

SET default_with_oids = false;

--
-- Name: account; Type: TABLE; Schema: public; Owner: valentine
--

CREATE TABLE public.account (
    name character varying(40) NOT NULL,
    email character varying(40) NOT NULL,
    password character varying(256) NOT NULL,
    id character(36) NOT NULL
);


ALTER TABLE public.account OWNER TO valentine;

--
-- Name: account account_pkey; Type: CONSTRAINT; Schema: public; Owner: valentine
--

ALTER TABLE ONLY public.account
    ADD CONSTRAINT account_pkey PRIMARY KEY(id);


--
-- PostgreSQL database dump complete
--
