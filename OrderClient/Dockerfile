FROM mcr.microsoft.com/dotnet/core/runtime:3.0-buster-slim AS base
WORKDIR /app

FROM mcr.microsoft.com/dotnet/core/sdk:3.0-buster AS build
WORKDIR /src
COPY ["protos/", "protos/"]
COPY ["OrderClient/OrderClient.csproj", "OrderClient/"]
COPY ["OrderClient/nuget.config", "OrderClient/"]
RUN dotnet restore "OrderClient/OrderClient.csproj"
COPY . .
WORKDIR "/src/OrderClient"
RUN dotnet build "OrderClient.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "OrderClient.csproj" -o /app/publish -c Release -r linux-x64 --self-contained true

FROM jlesage/baseimage-gui:debian-9 AS final
COPY --from=publish /app/publish /app
RUN echo '#!/bin/sh\n\
exec /app/OrderClient'\
>> /startapp.sh
RUN chmod u+x /startapp.sh && chmod a+x /app/OrderClient
ENV APP_NAME="OrderClient"