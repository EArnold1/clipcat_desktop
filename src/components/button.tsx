import clsx from 'clsx';

type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'danger' | 'plain';
type ButtonRounded = 'sm' | 'md' | 'lg' | 'xl';
type ButtonSize = 'sm' | 'md' | 'lg';

export interface ButtonProps
  extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  children: React.ReactNode;
  variant?: ButtonVariant;
  loading?: boolean;
  rounded?: ButtonRounded;
  size?: ButtonSize;
}

const roundedVariant: Record<ButtonRounded, string> = {
  xl: 'rounded-xl',
  lg: 'rounded-lg',
  md: 'rounded-md',
  sm: 'rounded-sm',
};

const colorVariants: Record<ButtonVariant, string> = {
  primary: 'bg-blue-600 text-white hover:bg-blue-700 border border-blue-900',
  secondary:
    'bg-gray-200 text-gray-800 hover:bg-gray-300 border border-gray-400',
  outline: 'border border-gray-400 text-gray-700 hover:bg-gray-100',
  danger: 'bg-red-600 text-white hover:bg-red-700',
  plain: 'bg-transparent',
};

const sizeVariants: Record<ButtonSize, string> = {
  sm: 'px-2 py-1 text-xs',
  md: 'px-4 py-2 text-base',
  lg: 'px-6 py-3 text-lg',
};

export const buttonVariants = {
  sizeVariants,
  roundedVariant,
  colorVariants,
};

export const Button = ({
  children,
  variant = 'primary',
  className,
  disabled,
  rounded = 'md',
  size = 'md',
  loading = false,
  ...props
}: ButtonProps) => {
  return (
    <button
      disabled={disabled || loading}
      className={clsx(
        'font-medium focus:outline-none cursor-pointer',
        buttonVariants.colorVariants[variant],
        buttonVariants.roundedVariant[rounded],
        buttonVariants.sizeVariants[size],
        (disabled || loading) && 'opacity-50 cursor-not-allowed',
        className
      )}
      {...props}
    >
      {children}
    </button>
  );
};
