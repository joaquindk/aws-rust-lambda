FROM amazon/aws-lambda-provided

RUN yum -y update && yum clean all
RUN yum -y install git gcc curl openssl openssl-devel ca-certificates tar && yum clean all

RUN curl https://sh.rustup.rs -sSf | sh -s -- --no-modify-path --default-toolchain stable -y

ENV PATH "$PATH:/root/.cargo/bin"
RUN echo "$PATH"
RUN rustup --version
RUN cargo --version
RUN rustc --version
    