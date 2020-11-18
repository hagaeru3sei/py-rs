FROM rust:1.47

WORKDIR /opt/app
COPY . .

#RUN cargo install --path .
#
#CMD ["myapp"]

RUN apt-get update && apt-get upgrade -y \
 && apt-get install -y \
    git \
    make \
    build-essential \
    libssl-dev \
    zlib1g-dev \
    libbz2-dev \
    libreadline-dev \
    libsqlite3-dev \
    wget \
    curl \
    llvm \
    libncurses5-dev \
    libncursesw5-dev \
    xz-utils \
    tk-dev \
    libffi-dev \
    liblzma-dev \
    vim \
    python3-pip \
    python3-setuptools
RUN python3 -m pip install -U --upgrade pip wheel six
RUN python3 -m pip install -U numpy scipy pandas joblib scikit-learn tensorflow

CMD ["sleep", "3600"]