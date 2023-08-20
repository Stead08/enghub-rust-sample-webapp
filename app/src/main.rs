use diesel::r2d2::ConnectionManager;
use tonic::transport::Server;

use app_context::AppContext;
use grpc_handler::user::v1::user_service_server::UserServiceServer;
use grpc_handler::UserServiceHandler;
use repository_impl::UserRepositoryImpl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    // DBに接続、コネクションプールを作成
    let database_url =
        std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL");
    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::new(manager).expect("failed to create the connection pool");

    // UserRepositoryを作成する
    let user_repository = UserRepositoryImpl::new(pool);

    // AppContextにUserRepositoryの実装を持たせる
    let context = AppContext { user_repository };

    // gRPCのハンドラを作成する
    let user_service = UserServiceHandler::new(context);

    let addr = "[::1]:50001".parse()?;
    println!("Start sample app server!");

    // gRPCのハンドラを登録してサーバを起動する
    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(addr)
        .await?;

    Ok(())
}
